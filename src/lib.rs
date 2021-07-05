/*
Written by Whanos
https://twitter.com/WhanosSergal
https://github.com/Whanos
I know this code is shoddy. This is my first real attempt at writing a library
*/

use std::io::Read;
use serde_json::{Value};
use serde::{Serialize, Deserialize};

struct FileData {
    width: u32,
    height: u32,
    ext: String,
    size: u32,
    md5: String,
    url: String
}

struct PreviewData {
    width: u32,
    height: u32,
    url: String
    // Note. An API request tells me there's an "alternatives" field, but I don't know what it does.
    // also the api docs say nothing about it. oh well!
}

struct SampleData {
    has: bool,
    height: u32,
    width: u32,
    url: String
}

struct ScoreData {
    up: u32,
    down: u32,
    total: u32
}
#[derive(Deserialize)]
struct TagsData {
    general: Vec<String>,
    species: Vec<String>,
    character: Vec<String>,
    copyright: Vec<String>,
    artist: Vec<String>,
    invalid: Vec<String>,
    lore: Vec<String>,
    meta: Vec<String>
}

struct FlagsData {
    pending: bool,
    flagged: bool,
    note_locked: bool,
    status_locked: bool,
    rating_locked: bool,
    deleted: bool
}

struct RelationshipsData {
    parent_id: u32,
    has_children: bool,
    has_active_children: bool,
    children: Vec<u32>
}

// i know this sucks
pub struct PostInfo {
    id: u32,
    created_at: String,
    updated_at: String,
    file: FileData,
    preview: PreviewData,
    sample: SampleData,
    score: ScoreData,
    tags: TagsData,
    locked_tags: Vec<String>,
    change_seq: u32,
    flags: FlagsData,
    rating: String,
    fav_count: u32,
    sources: Vec<String>,
    pools: Vec<String>,
    relationships: RelationshipsData,
    approver_id: u32,
    uploader_id: u32,
    description: String,
    comment_count: u32,
    is_favourited: bool,
    has_notes: bool,
    // duration: idk lol, api docs say nothin
}


//i know this is synchronous. async when i can figure it out, lol :(
pub fn get_post_info(id: &str, user_agent: &str) -> PostInfo {
    let url = format!("https://e621.net/posts/{}.json", id);
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(user_agent);
    let built_client = client.build().unwrap();
    let mut res = built_client.get(url).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body);
    //oh god.
    let v: Value = serde_json::from_str(body.as_str()).unwrap();

    //ok lets struct it :)
    //i know this code sucks lmfao
    //but it works!

    let general = v["post"]["tags"].as_str().unwrap();
    let god_help_us: TagsData = serde_json::from_str(&general).unwrap();

    let post_info = PostInfo {
        id: v["post"]["id"].as_str().unwrap().parse::<u32>().unwrap(),
        created_at: String::from(v["post"]["created_at"].as_str().unwrap()),
        updated_at: String::from(v["post"]["updated_at"].as_str().unwrap()),
        file: FileData {
            width: v["post"]["file"]["width"].as_str().unwrap().parse::<u32>().unwrap(),
            height: v["post"]["file"]["height"].as_str().unwrap().parse::<u32>().unwrap(),
            ext: String::from(v["post"]["file"]["ext"].as_str().unwrap()),
            size: v["post"]["file"]["size"].as_str().unwrap().parse::<u32>().unwrap(),
            md5: String::from(v["post"]["file"]["md5"].as_str().unwrap()),
            url: String::from(v["post"]["file"]["url"].as_str().unwrap())
        },
        preview: PreviewData {
            width: v["post"]["preview"]["width"].as_str().unwrap().parse::<u32>().unwrap(),
            height: v["post"]["preview"]["height"].as_str().unwrap().parse::<u32>().unwrap(),
            url: String::from(v["post"]["preview"]["url"].as_str().unwrap())
        },
        sample: SampleData {
            has: v["post"]["sample"]["has"].as_str().unwrap().parse::<bool>().unwrap(),
            height: v["post"]["sample"]["height"].as_str().unwrap().parse::<u32>().unwrap(),
            width: v["post"]["sample"]["width"].as_str().unwrap().parse::<u32>().unwrap(),
            url: String::from(v["post"]["sample"]["url"].as_str().unwrap())
        },
        score: ScoreData {
            up: v["post"]["score"]["up"].as_str().unwrap().parse::<u32>().unwrap(),
            down: v["post"]["score"]["down"].as_str().unwrap().parse::<u32>().unwrap(),
            total: v["post"]["score"]["total"].as_str().unwrap().parse::<u32>().unwrap()
        },
        tags: god_help_us,
        locked_tags: vec![],
        change_seq: 0,
        flags: FlagsData {
            pending: false,
            flagged: false,
            note_locked: false,
            status_locked: false,
            rating_locked: false,
            deleted: false
        },
        rating: "".to_string(),
        fav_count: 0,
        sources: vec![],
        pools: vec![],
        relationships: RelationshipsData {
            parent_id: 0,
            has_children: false,
            has_active_children: false,
            children: vec![]
        },
        approver_id: 0,
        uploader_id: 0,
        description: "".to_string(),
        comment_count: 0,
        is_favourited: false,
        has_notes: false
    };

    post_info
}