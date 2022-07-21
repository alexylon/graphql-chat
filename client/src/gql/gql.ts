import {gql} from "@apollo/client";


const _schema = gql`
    type Query {
        getChats: [Chat!]!
    }

    type Chat {
        id: Float!
        message: String!
        name: String!
    }

    type Mutation {
        createChat(message: String!, name: String!): Chat!
    }

    type Subscription {
        messageSent: Chat!
    }
`;

// // warp-server
// const schema = gql`
//     type Chat {
//         id: String!
//         name: String!
//         message: String!
//     }
//
//     type ChatChanged {
//         mutationType: MutationType!
//         id: ID!
//         chat: Chat
//     }
//
//     type Mutation {
//         createChat(name: String!, message: String!): [Chat!]!
//     }
//
//     enum MutationType {
//         CREATED
//     }
//
//     type Query {
//         getChats: [Chat!]!
//     }
//
//     type Subscription {
//         messageSent(mutationType: MutationType): ChatChanged!
//     }
// `;

export const ALL_CHATS = gql`
    query allChats {
        getChats {
            id
            name
            message
        }
    }
`;

export const CHATS_SUBSCRIPTION = gql`
    subscription OnNewChat {
        messageSent {
            id
            name
            message
        }
    }
`;

// // warp-server
// export const CHATS_SUBSCRIPTION = gql`
//     subscription OnNewChat {
//         messageSent {
//             id
//             chat {
//                 id
//                 name
//                 message
//             }
//             chats {
//                 id
//                 name
//                 message
//             }
//         }
//     }
// `;

export const SEND_MESSAGE = gql`
    mutation createChat($name: String!, $message: String!) {
        createChat(name: $name, message: $message) {
            id
            name
            message
        }
    }
`;


