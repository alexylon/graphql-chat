mod simple_broker;

use std::{sync::Arc, time::Duration};

use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID, Data};
use futures_util::{lock::Mutex, Stream, StreamExt};
use simple_broker::SimpleBroker;
use slab::Slab;
use serde::Deserialize;

pub type ChatsSchema = Schema<Query, Mutation, Subscription>;

#[derive(Clone)]
pub struct Chat {
    id: ID,
    name: String,
    message: String,
}

#[Object]
impl Chat {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn message(&self) -> &str {
        &self.message
    }
}

pub type Storage = Arc<Mutex<Slab<Chat>>>;

pub struct Query;

#[Object]
impl Query {
    async fn get_chats(&self, ctx: &Context<'_>) -> Vec<Chat> {
        let chats = ctx.data_unchecked::<Storage>().lock().await;
        chats.iter().map(|(_, chat)| chat).cloned().collect()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_chat(&self, ctx: &Context<'_>, name: String, message: String) -> Vec<Chat> {
        let mut chats = ctx.data_unchecked::<Storage>().lock().await;
        let entry = chats.vacant_entry();
        let id: ID = entry.key().into();
        let chat = Chat {
            id: id.clone(),
            name,
            message,
        };
        entry.insert(chat.clone());
        SimpleBroker::publish(ChatChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
            chat: Some(chat),
            chats: None,
        });
        chats.iter().map(|(_, chat)| chat).cloned().collect()
    }
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
enum MutationType {
    Created,
}

#[derive(Clone)]
struct ChatChanged {
    mutation_type: MutationType,
    id: ID,
    chat: Option<Chat>,
    chats: Option<Vec<Chat>>,
}

#[Object]
impl ChatChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &ID {
        &self.id
    }

    async fn chat(&self, ctx: &Context<'_>) -> Result<Option<Chat>> {
        let chats = ctx.data_unchecked::<Storage>().lock().await;
        let id = self.id.parse::<usize>()?;
        Ok(chats.get(id).cloned())
    }

    async fn chats(&self, ctx: &Context<'_>) -> Result<Vec<Chat>> {
        let chats = ctx.data_unchecked::<Storage>().lock().await;
        Ok(chats.iter().map(|(_, chat)| chat).cloned().collect())
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn message_sent(&self, _ctx: &Context<'_>) -> impl Stream<Item=Vec<Chat>> {
        // let chats = ctx.data_unchecked::<Storage>().lock().await;
        // let chats_vec: Vec<Chat> = chats.iter().map(|(_, chat)| chat).cloned().collect();
        // async_stream::stream! {
        //         yield chats_vec;
        // }
        SimpleBroker::<Vec<Chat>>::subscribe().filter(move |_event| {
            async move { true }
        })
    }
}

pub type TokenSchema = Schema<Query, Mutation, Subscription>;

pub struct Token(pub String);

pub async fn on_connection_init(value: serde_json::Value) -> Result<Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = Data::default();
        data.insert(Token(payload.token));
        Ok(data)
    } else {
        Err("Token is required".into())
    }
}
