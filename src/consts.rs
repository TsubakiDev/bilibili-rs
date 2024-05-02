use std::collections::HashMap;

const BILI_API_URL: &str = "https://api.bilibili.com/";
const BILI_MAIN_URL: &str = "https://www.bilibili.com/";
const BILI_PASSPORT_URL: &str = "https://passport.bilibili.com/";
const BILI_ELEC_URL: &str = "https://elec.bilibili.com/";
const BILI_LIVE_URL: &str = "https://api.live.bilibili.com/";
const BILI_VC_URL: &str = "https://api.vc.bilibili.com/";

const USER_AGENTS: [&str; 6] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36",
    "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/44.0.2403.155 Safari/537.36",
    "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2227.1 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2227.0 Safari/537.36",
    "Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.2.20) Gecko/20110804 Red Hat/3.6-2.el5 Firefox/3.6.20",
];

struct VideoZone {
    name: &'static str,
    code: &'static str,
    desc: &'static str,
}

const VIDEO_ZONES: [(i32, &'static str, &'static str, &'static str); 3] = [
    (1, "动画", "douga", ""),
    (24, "MAD·AMV", "mad", "具有一定制作程度的动画或静画的二次创作视频"),
    (25, "MMD·3D", "mmd", "使用MMD(MikuMikuDance)和其他3D建模类软件制作的视频"),
    // TODO: More mappings
];

lazy_static::lazy_static! {
    static ref VIDEO_ZONE_MAP: HashMap<i32, VideoZone> = {
        let mut map = HashMap::new();
        for &(tid, name, code, desc) in &VIDEO_ZONES {
            map.insert(tid, VideoZone::new(name, code, desc));
        }
        map
    };
}

impl VideoZone {
    fn new(name: &'static str, code: &'static str, desc: &'static str) -> Self {
        VideoZone { name, code, desc }
    }
}


fn get_video_zone(tid: i32) -> Option<&'static VideoZone> {
    VIDEO_ZONE_MAP.get(&tid)
}