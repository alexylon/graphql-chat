# GraphQL Live Chat App
A simple live GraphQL chat app built using React, Node, Apollo Server/Client and TypeGraphQL.

## To Run
### Server
- Run `npm i` to install deps
- Run `npm run watch` to compile TS files
- In another terminal, run `npm run dev` to run the server
- Visit `localhost:9000/graphql` to view GraphQL Playground

### Client
- Run `npm i` to install deps
- Run `npm start` to start the client
- Open two browser tabs or more at `localhost:3000`, enter names and start chatting between them.

## GraphQL API Endpoints
- `/graphql`: has `createChat` mutation and `allChats` query resolvers
- `/subscriptions`: has `messageSent` subscription

Used the following [tutorial](https://dev.to/dsckiitdev/build-a-chat-app-with-graphql-subscriptions-typescript-part-3-30dd) as a starting point.

[![forthebadge](https://forthebadge.com/images/badges/made-with-typescript.svg)](https://forthebadge.com)