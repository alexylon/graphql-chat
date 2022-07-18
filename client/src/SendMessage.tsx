import {useState, FC} from "react";
import {gql, useMutation} from "@apollo/client";
import {Button, Stack, TextField} from '@mui/material';
import {SEND_MESSAGE} from "./gql/gql";


interface SendMessageProps {
    name: string;
}

const SendMessage: FC<SendMessageProps> = ({name}) => {
    const [input, setInput] = useState<string>("");
    const [sendMessage] = useMutation(SEND_MESSAGE);

    const handleSend = () => {
        sendMessage({variables: {name: name, message: input}})
            .then((data) => {
                console.log(data);
                setInput("");
            })
            .catch((err) => console.log(err));
    };

    return (
        <Stack spacing={2} direction="row">
            <TextField
                id="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                label="Type message"
                variant="outlined"/>
            <Button variant="contained" onClick={handleSend}>Send Message</Button>
        </Stack>
    );
};

export default SendMessage;
