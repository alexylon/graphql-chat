import {ApolloClient, InMemoryCache} from "@apollo/client";
import {ApolloProvider} from "@apollo/client";
import {WebSocketLink} from "@apollo/client/link/ws";
import {split, HttpLink} from "@apollo/client";
import {getMainDefinition} from "@apollo/client/utilities";
import Chats from "./Chats";
import SendMessage from "./SendMessage";
import {useState} from "react";
import {Box, Button, Stack, TextField} from '@mui/material';

const wsLink = new WebSocketLink({
    uri: "ws://localhost:9000/subscriptions",
    options: {
        reconnect: true,
    },
});

const httpLink = new HttpLink({
    uri: "http://localhost:9000/graphql",
    credentials: "include",
});

// // warp-server
// const wsLink = new WebSocketLink({
//     uri: "ws://localhost:8000",
//     options: {
//         reconnect: true,
//     },
// });
//
// // warp-server
// const httpLink = new HttpLink({
//     uri: "http://localhost:8000",
//     credentials: "include",
// });

const link = split(
    ({query}) => {
        const definition = getMainDefinition(query);
        return (
            definition.kind === "OperationDefinition" &&
            definition.operation === "subscription"
        );
    },
    wsLink,
    httpLink
);

const client = new ApolloClient({
    link,
    cache: new InMemoryCache(),
});

const App = () => {
    const [name, setName] = useState<string>("");
    const [entered, setEntered] = useState<boolean>(false);

    return (
        <ApolloProvider client={client}>
            <Box marginLeft={3} marginTop={3}>
                {!entered && (
                    <Stack spacing={2} direction="row">
                        <TextField
                            id="name"
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            label="Name"
                            variant="outlined"/>
                        <Button variant="contained" onClick={() => setEntered(true)}>Enter chat</Button>
                    </Stack>
                )}

                {name !== "" && entered && (
                    <div>
                        <h3>{name}</h3>
                        <Chats/>
                        <SendMessage name={name}/>
                    </div>
                )}
            </Box>
        </ApolloProvider>
    );
};

export default App;
