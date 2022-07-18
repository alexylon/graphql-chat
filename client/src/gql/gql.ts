import {gql} from "@apollo/client";


const schema = gql`
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

export const SEND_MESSAGE = gql`
    mutation createChat($name: String!, $message: String!) {
        createChat(name: $name, message: $message) {
            id
            name
            message
        }
    }
`;


