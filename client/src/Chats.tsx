import {useQuery} from "@apollo/client";
import {useEffect} from "react";
import {ALL_CHATS, CHATS_SUBSCRIPTION} from "./gql/gql";


const Chats = () => {
    const {loading, error, data, subscribeToMore} = useQuery(ALL_CHATS);

    useEffect(() => {
        subscribeToMore({
            document: CHATS_SUBSCRIPTION,
            updateQuery: (prev, {subscriptionData}) => {
                if (!subscriptionData.data) return prev;
                const newChat = subscriptionData.data.messageSent;

                return {
                    getChats: [...prev.getChats, newChat],
                };
            },
        });
    }, [subscribeToMore]);

    if (loading) return <p>Loading...</p>;
    if (error) return <p>`Error! ${error.message}`</p>;

    return (
        <div>
            {data.getChats.map((chat: any) => (
                <div key={chat.id}>
                    <p>
                        <b>{chat.name}:</b> {chat.message}
                    </p>
                </div>
            ))}
        </div>
    );
};

export default Chats;
