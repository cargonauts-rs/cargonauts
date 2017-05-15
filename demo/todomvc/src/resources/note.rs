use chrono::{DateTime, UTC};

use cargonauts::api::{Resource, Index, Post, Patch, Delete, Environment, Error};
use cargonauts::futures::{Future};
use uuid::Uuid;

use clients::RedisStore;

#[derive(ApiSerialize, Serialize, Deserialize, Clone)]
pub struct Note {
    #[api_id]
    id: Uuid,
    text: String,
    completed: bool,
    created_at: DateTime<UTC>,
}

#[derive(ApiDeserialize)]
pub struct PostNote {
    text: String,
    completed: bool,
    created_at: Option<DateTime<UTC>>,
}

#[derive(ApiDeserialize)]
#[ignore_api_id]
pub struct PatchNote {
    text: Option<String>,
    completed: Option<bool>,
    created_at: Option<DateTime<UTC>>,
}

impl Resource for Note {
    type Identifier = Uuid;
}

impl Index for Note {
    fn index(env: &Environment) -> Box<Future<Item = Vec<Note>, Error = Error>> {
        let future = env.client::<RedisStore>()
                        .and_then(|store| store.get_all());
        Box::new(future)
    }
}

impl Post for Note {
    type Post = PostNote;

    fn post(post: PostNote, env: &Environment) -> Box<Future<Item = Note, Error = Error>> {
        let id = Uuid::new_v4();
        let created_at = post.created_at.unwrap_or_else(UTC::now);
        let note = Note {
            id, created_at,
            text: post.text,
            completed: post.completed,
        };
        let future = env.client::<RedisStore>()
                        .and_then(move |store| { store.save(id, note.clone()).map(|_| note) });
        Box::new(future)
    }
}

impl Patch for Note {
    type Patch = PatchNote;

    fn patch(id: Uuid, patch: PatchNote, env: &Environment) -> Box<Future<Item = Note, Error = Error>> {
        let future = env.client::<RedisStore>().and_then(move |store| {
            store.get::<Note>(id).and_then(move |mut note| {
                patch.text.map(|text| note.text = text);
                patch.completed.map(|completed| note.completed = completed);
                patch.created_at.map(|created_at| note.created_at = created_at);
                store.save(id, note.clone()).map(|_| note)
            })
        });
        Box::new(future)
    }
}

impl Delete for Note {
    fn delete(id: Uuid, env: &Environment) -> Box<Future<Item = (), Error = Error>> {
        let future = env.client::<RedisStore>()
                        .and_then(move |store| store.delete(id));
        Box::new(future)
    }
}
