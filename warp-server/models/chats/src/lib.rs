mod simple_broker;

use std::{sync::Arc};

use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};
use futures_util::{lock::Mutex, Stream, StreamExt};
use simple_broker::SimpleBroker;
use slab::Slab;
// use serde::Deserialize;

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
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn message_sent(&self, mutation_type: Option<MutationType>) -> impl Stream<Item=ChatChanged> {
        SimpleBroker::<ChatChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                println!("mutation_type: {}", event.mutation_type == mutation_type);
                event.mutation_type == mutation_type
            } else {
                println!("else");
                true
            };
            println!("res: {}", res);
            async move { res }
        })
    }
}
