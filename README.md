# GraphQL Live Chat App
A simple live GraphQL chat app built using React, Node, Apollo Server/Client and TypeGraphQL.
There is an option to use Rust async-graphql running on warp server.

## To Run
### Server
- Run `npm i` to install deps
- Run `npm run watch` to compile TS files
- In another terminal, run `npm run dev` to start the server
- Visit `localhost:9000/graphql` to view GraphQL Playground

### Client
- Run `npm i` to install deps
- Run `npm start` to start the client
- Open two browser tabs or more at `localhost:3000`, enter names and start chatting between them.

## GraphQL API Endpoints
- `/graphql`: has `createChat` mutation and `allChats` query resolvers
- `/subscriptions`: has `messageSent` subscription


[![forthebadge](https://forthebadge.com/images/badges/made-with-typescript.svg)](https://forthebadge.com)

OR

### Warp Server
Instead of running the TS server you can run Rust warp-server with
- In the client uncomment all "warp-server" and comment the corresponding lines in App.tsx, Chat,tsx and gql.ts
- Run `cargo run` to start the server
- Run the client (see above)
- Visit `localhost:8000` to view GraphQL Playground. All queries, mutations and subscriptions are on `/`.


[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

Used the following [tutorial](https://dev.to/dsckiitdev/build-a-chat-app-with-graphql-subscriptions-typescript-part-1-2p70) as a starting point.

