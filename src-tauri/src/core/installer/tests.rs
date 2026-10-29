use super::*;
use serde_json::json;

#[test]
fn duplicate_assets_are_downloaded_once_and_invalid_metadata_is_rejected() {
    let hash = "a".repeat(40);
    let index: AssetIndex = serde_json::from_value(
        json!({"objects":{"a.ogg":{"hash":hash,"size":2},"b.ogg":{"hash":hash,"size":2}}}),
    )
    .unwrap();
    let mut plan = InstallPlan::new();
    plan.assets(
        &index,
        Path::new("repo"),
        "https://resources.download.minecraft.net",
    )
    .unwrap();
    assert_eq!(plan.files.len(), 1);
    let bad: AssetIndex =
        serde_json::from_value(json!({"objects":{"../escape":{"hash":"x","size":1}}})).unwrap();
    assert!(
        plan.assets(&bad, Path::new("repo"), "https://example.org")
            .is_err()
    );
    let bad: AssetIndex =
        serde_json::from_value(json!({"objects":{"asset":{"hash":"x","size":1}}})).unwrap();
    assert!(
        plan.assets(&bad, Path::new("repo"), "https://example.org")
            .is_err()
    );
    let mut conflict = plan.files[0].info.clone();
    conflict.size = 3;
    assert!(
        plan.add(3, &conflict, plan.files[0].out_path.clone())
            .is_err()
    );
}

#[test]
fn failed_installations_clean_only_their_own_directory() {
    let root = std::env::temp_dir().join(format!("pcl-install-test-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    let directory = InstallDirectory::create(&root, "custom").unwrap();
    fs::write(directory.path.join("world.dat"), "keep").unwrap();
    assert!(InstallDirectory::create(&root, "custom").is_err());
    assert_eq!(
        fs::read_to_string(directory.path.join("world.dat")).unwrap(),
        "keep"
    );
    drop(directory);
    assert!(!root.join("versions/custom").exists());
    fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
#[ignore = "Downloads an official Minecraft installation; run explicitly"]
async fn official_vanilla_install_with_custom_name() {
    let root = std::env::temp_dir().join(format!("pcl-install-smoke-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    let client = MinecraftApiClient::new(
        reqwest::Client::new(),
        &super::super::api_client::ApiProvider::Official,
    );
    let result = install_version(
        &client,
        &root,
        "1.2.5",
        "Custom Vanilla",
        0,
        &Channel::new(|_| Ok(())),
    )
    .await;
    if result.is_ok() {
        let repo = super::super::repository::GameRepository::new("test", root.clone());
        let instances = repo.game_instances();
        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].name, "Custom Vanilla");
        assert_eq!(instances[0].version, "1.2.5");
        assert!(instances[0].jar_path.is_file());
    }
    fs::remove_dir_all(root).unwrap();
    result.unwrap();
}

use sha1::Digest;
use std::{
    io::{Read, Write},
    sync::atomic::{AtomicBool, Ordering},
};

struct MockMirror {
    url: &'static str,
    stop: Arc<AtomicBool>,
    requests: Arc<std::sync::Mutex<Vec<String>>>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl MockMirror {
    fn new(files: HashMap<String, Vec<u8>>, transient_client_error: bool) -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = Box::leak(format!("http://{}", listener.local_addr().unwrap()).into_boxed_str());
        let stop = Arc::new(AtomicBool::new(false));
        let stopped = stop.clone();
        let requests = Arc::new(std::sync::Mutex::new(Vec::new()));
        let recorded = requests.clone();
        let worker = std::thread::spawn(move || {
            let mut fail_client = transient_client_error;
            while !stopped.load(Ordering::Relaxed) {
                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(2));
                        continue;
                    }
                    Err(error) => panic!("Mock accept failed: {error}"),
                };
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                    .unwrap();
                let mut buffer = [0; 8192];
                let count = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..count]);
                let path = request.split_whitespace().nth(1).unwrap().to_owned();
                recorded.lock().unwrap().push(path.clone());
                let (status, body) = if path == "/client.jar" && fail_client {
                    fail_client = false;
                    ("503 Service Unavailable", b"retry".as_slice())
                } else if let Some(body) = files.get(&path) {
                    ("200 OK", body.as_slice())
                } else {
                    ("404 Not Found", b"missing".as_slice())
                };
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                let _ = stream.write_all(body);
            }
        });
        Self {
            url,
            stop,
            requests,
            worker: Some(worker),
        }
    }

    fn client(&self) -> MinecraftApiClient {
        let mut bases = super::super::api_client::ApiBases::new(
            &super::super::api_client::ApiProvider::Official,
        );
        bases.meta_base = self.url;
        bases.resources_base = Box::leak(format!("{}/assets", self.url).into_boxed_str());
        MinecraftApiClient::with_test_bases(bases)
    }

    fn count(&self, path: &str) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.as_str() == path)
            .count()
    }
}

impl Drop for MockMirror {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let result = self.worker.take().unwrap().join();
        if !std::thread::panicking() {
            result.unwrap();
        }
    }
}

fn fixture_archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut archive = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, bytes) in entries {
        archive
            .start_file(*name, zip::write::FileOptions::default())
            .unwrap();
        archive.write_all(bytes).unwrap();
    }
    archive.finish().unwrap().into_inner()
}

fn fixture_files(bad_client_hash: bool) -> HashMap<String, Vec<u8>> {
    let hash = |bytes: &[u8]| format!("{:x}", sha1::Sha1::digest(bytes));
    let info = |url: &str, bytes: &[u8]| json!({"sha1":hash(bytes), "size":bytes.len(), "url":url});
    let jar = b"verified client";
    let asset = b"sound";
    let asset_hash = hash(asset);
    let natives = fixture_archive(&[
        ("libtest.dylib", b"native"),
        ("META-INF/MANIFEST.MF", b"skip"),
        ("excluded/ignored", b"skip"),
    ]);
    let index = serde_json::to_vec(&json!({"virtual":true,"map_to_resources":true,"objects":{
        "sounds/a.ogg":{"hash":asset_hash,"size":asset.len()},
        "sounds/duplicate.ogg":{"hash":asset_hash,"size":asset.len()}
    }}))
    .unwrap();
    let mut artifact = info(
        "https://libraries.minecraft.net/test/library.jar",
        b"library",
    );
    artifact["path"] = "test/library.jar".into();
    let mut native_info = info("https://libraries.minecraft.net/test/native.jar", &natives);
    native_info["path"] = "test/native.jar".into();
    let mut client = info("https://piston-data.mojang.com/client.jar", jar);
    if bad_client_hash {
        client["sha1"] = "0".repeat(40).into();
    }
    let metadata = serde_json::to_vec(&json!({
        "id":"1.test", "type":"release", "assets":"test-index", "mainClass":"net.minecraft.client.main.Main",
        "downloads":{"client":client},
        "assetIndex":{"id":"test-index", "sha1":hash(&index), "size":index.len(), "url":"https://piston-meta.mojang.com/index.json"},
        "libraries":[
            {"name":"test:library:1", "downloads":{"artifact":artifact}},
            {"name":"test:native:1", "natives":{(RuleContext::current().os):"natives-test"}, "extract":{"exclude":["excluded/"]}, "downloads":{"classifiers":{"natives-test":native_info}}},
            {"name":"test:skip:1", "rules":[{"action":"disallow"}], "downloads":{"artifact":{"sha1":"0".repeat(40),"size":1,"url":"https://libraries.minecraft.net/forbidden.jar","path":"forbidden.jar"}}}
        ],
        "logging":{"client":{"argument":"-Dlog4j.configurationFile=${path}", "file":{"id":"client.xml","sha1":hash(b"<xml/>"),"size":6,"url":"https://piston-data.mojang.com/log.xml"}}},
        "javaVersion":{"majorVersion":21}, "arguments":{"game":["--fixture","preserved"]}
    })).unwrap();
    let manifest = serde_json::to_vec(&json!({"latest":{"release":"1.test","snapshot":"1.test"},"versions":[{
        "id":"1.test","type":"release","time":"2026-01-01T00:00:00Z","releaseTime":"2026-01-01T00:00:00Z",
        "url":"https://piston-meta.mojang.com/version.json","sha1":hash(&metadata)
    }]})).unwrap();
    [
        ("/mc/game/version_manifest_v2.json".into(), manifest),
        ("/version.json".into(), metadata),
        ("/index.json".into(), index),
        ("/client.jar".into(), jar.to_vec()),
        ("/maven/test/library.jar".into(), b"library".to_vec()),
        ("/maven/test/native.jar".into(), natives),
        ("/log.xml".into(), b"<xml/>".to_vec()),
        (
            format!("/assets/{}/{}", &asset_hash[..2], asset_hash),
            asset.to_vec(),
        ),
    ]
    .into()
}

#[tokio::test]
async fn full_install_retries_publishes_custom_name_and_reuses_verified_cache() {
    let mirror = MockMirror::new(fixture_files(false), true);
    let client = mirror.client();
    let root = std::env::temp_dir().join(format!("pcl-full-install-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    let repo = super::super::repository::GameRepository::new("test", root.clone());
    assert!(repo.game_instances().is_empty());
    let reports = Arc::new(std::sync::Mutex::new(Vec::new()));
    let events = reports.clone();
    let channel = Channel::new(move |event| {
        events.lock().unwrap().push(format!("{event:?}"));
        Ok(())
    });
    install_version(&client, &root, "1.test", "Custom Vanilla", 7, &channel)
        .await
        .unwrap();
    let path = root.join("versions/Custom Vanilla");
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(path.join("Custom Vanilla.json")).unwrap()).unwrap();
    assert_eq!(value["id"], "Custom Vanilla");
    assert_eq!(value["clientVersion"], "1.test");
    assert_eq!(value["javaVersion"]["majorVersion"], 21);
    assert_eq!(value["arguments"]["game"][0], "--fixture");
    assert_eq!(
        fs::read(path.join("natives/libtest.dylib")).unwrap(),
        b"native"
    );
    assert!(!path.join("natives/META-INF").exists());
    assert!(!path.join("natives/excluded").exists());
    assert_eq!(
        fs::read(path.join("resources/sounds/a.ogg")).unwrap(),
        b"sound"
    );
    assert_eq!(
        fs::read(root.join("assets/virtual/test-index/sounds/duplicate.ogg")).unwrap(),
        b"sound"
    );
    let instances = repo.game_instances();
    assert_eq!(instances.len(), 1, "the same repository sees new installs");
    assert_eq!(instances[0].version, "1.test");
    assert_eq!(mirror.count("/client.jar"), 2, "503 is retried");
    assert_eq!(mirror.count("/forbidden.jar"), 0);
    assert!(
        install_version(&client, &root, "1.test", "Custom Vanilla", 8, &channel)
            .await
            .is_err()
    );
    assert!(
        path.join("Custom Vanilla.json").is_file(),
        "existing installations survive conflicts"
    );
    install_version(
        &client,
        &root,
        "1.test",
        "Second",
        9,
        &Channel::new(|_| Err(tauri::Error::Io(std::io::Error::other("UI closed")))),
    )
    .await
    .unwrap();
    assert_eq!(repo.game_instances().len(), 2);
    assert_eq!(mirror.count("/maven/test/library.jar"), 1);
    assert_eq!(mirror.count("/maven/test/native.jar"), 1);
    assert_eq!(mirror.count("/index.json"), 1);
    assert_eq!(mirror.count("/log.xml"), 1);
    assert!(!reports.lock().unwrap().is_empty());
    fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn failed_install_never_publishes_an_instance_and_preserves_existing_data() {
    let mirror = MockMirror::new(fixture_files(true), false);
    let root = std::env::temp_dir().join(format!("pcl-failed-install-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    fs::write(root.join("keep.txt"), b"keep").unwrap();
    let result = install_version(
        &mirror.client(),
        &root,
        "1.test",
        "Broken",
        0,
        &Channel::new(|_| Ok(())),
    )
    .await;
    assert!(result.is_err());
    assert!(!root.join("versions/Broken").exists());
    assert_eq!(fs::read(root.join("keep.txt")).unwrap(), b"keep");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn native_extraction_refuses_zip_slip() {
    let root = std::env::temp_dir().join(format!("pcl-native-test-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    let path = root.join("bad.jar");
    fs::write(&path, fixture_archive(&[("../outside", b"bad")])).unwrap();
    assert!(
        extract_natives(
            &[NativeArchive {
                path,
                exclude: vec![]
            }],
            &root.join("natives")
        )
        .is_err()
    );
    assert!(!root.join("outside").exists());
    fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn failed_transfers_preserve_existing_files_and_remove_partial_files() {
    let mirror = MockMirror::new(fixture_files(false), false);
    let root = std::env::temp_dir().join(format!("pcl-transfer-test-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    let path = root.join("existing.jar");
    fs::write(&path, b"keep existing data").unwrap();
    let downloader = Downloader::default();
    let mut info = DownloadInfo {
        sha1: "0".repeat(40),
        size: b"verified client".len() as u64,
        url: format!("{}/client.jar", mirror.url),
        path: None,
    };
    assert!(downloader.download(&info, &path).await.is_err());
    assert_eq!(fs::read(&path).unwrap(), b"keep existing data");
    assert_eq!(
        fs::read_dir(&root).unwrap().count(),
        1,
        "failed .part files are removed"
    );
    assert_eq!(mirror.count("/client.jar"), 3);
    info.size = 1;
    assert!(downloader.download(&info, &path).await.is_err());
    assert_eq!(fs::read(&path).unwrap(), b"keep existing data");
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    info.sha1 = format!("{:x}", sha1::Sha1::digest(b"keep existing data"));
    info.size = b"keep existing data".len() as u64;
    downloader.download(&info, &path).await.unwrap();
    assert_eq!(
        mirror.count("/client.jar"),
        6,
        "verified cache does not access the network"
    );
    fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn tampered_version_json_fails_before_downloading_files() {
    let mut files = fixture_files(false);
    files.get_mut("/version.json").unwrap().push(b' ');
    let mirror = MockMirror::new(files, false);
    let root = std::env::temp_dir().join(format!("pcl-metadata-test-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    assert!(
        install_version(
            &mirror.client(),
            &root,
            "1.test",
            "Broken",
            0,
            &Channel::new(|_| Ok(()))
        )
        .await
        .is_err()
    );
    assert!(!root.join("versions/Broken").exists());
    assert_eq!(mirror.count("/index.json"), 0);
    fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
#[ignore = "Fetches official version JSON and asset indexes; run explicitly"]
async fn official_metadata_across_generations() {
    let client = MinecraftApiClient::new(
        reqwest::Client::new(),
        &super::super::api_client::ApiProvider::Official,
    );
    let root = std::env::temp_dir().join(format!("pcl-metadata-smoke-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&root).unwrap();
    let manifest: super::super::minecraft::VersionManifest = client
        .get_with_endpoint(super::super::minecraft::VERSION_MANIFEST_ENDPOINT, true)
        .await
        .unwrap();
    let downloader = Downloader::default();
    for id in [
        "rd-132211",
        "b1.7.3",
        "1.2.5",
        "1.16.5",
        "1.21.1",
        &manifest.latest.release,
    ] {
        let value = client.get_version_metadata(id).await.unwrap();
        let details: VersionDetails = serde_json::from_value(value).unwrap();
        let index_path = root.join(format!("{}.json", details.asset_index.id));
        downloader
            .download(&details.asset_index.download, &index_path)
            .await
            .unwrap();
        let index: AssetIndex = serde_json::from_slice(&fs::read(index_path).unwrap()).unwrap();
        let mut plan = InstallPlan::new();
        plan.add(1, &details.downloads.client, root.join("client.jar"))
            .unwrap();
        plan.libraries(&details, &root, &RuleContext::current())
            .unwrap();
        plan.assets(&index, &root, "https://resources.download.minecraft.net")
            .unwrap();
        println!(
            "{id}: {} downloads, {} legacy native archives, {} logical assets",
            plan.files.len(),
            plan.natives.len(),
            index.objects.len()
        );
    }
    fs::remove_dir_all(root).unwrap();
}
