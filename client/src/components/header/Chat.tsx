import React, { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import './Chat.css'

class Props {
    clicked: string = '';
    onClick: (type: string) => void = (type: string) => { };
}

class State {
    icon: string = '/src/assets/chat.png';
}

class Chat extends React.Component<Props, State> {
    constructor(props: any) {
        super(props);
        this.state = new State();
    }

    onClick = () => {
        this.props.onClick('chat');
    }

    componentDidMount(): void {
        if (this.props.clicked === 'chat') {
            this.setState({ icon: '/src/assets/selected.png' })
        }
    }

    componentDidUpdate(prevProps: Readonly<Props>, prevState: Readonly<State>, snapshot?: any): void {
        if (prevProps.clicked !== this.props.clicked) {
            if (this.props.clicked === 'chat') {
                this.setState({ icon: '/src/assets/selected.png' });
            } else {
                this.setState({ icon: '/src/assets/chat.png' });
            }
        }
    }

    render(): ReactNode {
        return (
            <div className="chat">
                <Link to='/'>
                    <img src={this.state.icon} alt="" onClick={this.onClick} />
                </Link>
            </div>
        )
    }
}

export default Chat;