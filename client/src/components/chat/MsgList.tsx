import React from "react";
import { Context, GlobalContext } from "../../context/GlobalContext";
import { Type } from "../../entity/msg";
import MsgListItem from "./MsgListItem";
import './MsgList.css';
import AddFriend from "./special/AddFriend";
import { UserInfo } from "../../service/user/userInfo";

class Props { }

class State {
    nickname: string = ''
}

class MsgList extends React.Component<Props, State> {
    listRef = React.createRef<HTMLDivElement>();

    static contextType = GlobalContext;

    constructor(props: Props) {
        super(props);
        this.state = new State();
    }

    componentDidMount = async () => {
        if (this.listRef.current) {
            this.listRef.current.scrollTop = this.listRef.current.scrollHeight;
        }
        let context = this.context as Context;
        let [_, nickname] = await UserInfo.avatarNickname(context.currentChatPeerId);
        this.setState({
            nickname: nickname
        })
    }

    componentDidUpdate(): void {
        if (this.listRef.current) {
            this.listRef.current.scrollTop = this.listRef.current.scrollHeight;
        }
    }

    render(): React.ReactNode {
        let context = this.context as Context;
        return (
            <div className="msg-list" ref={this.listRef}>
                {/* <div>LoadMore</div> */}
                {
                    context.currentChatMsgList.map((msg, index) => {
                        return <MsgListItem key={index} accountId={msg.head.sender} rawMsg={msg}/>
                    })
                }
            </div>
        )
    }
}

export default MsgList