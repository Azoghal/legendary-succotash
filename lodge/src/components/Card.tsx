interface ICardProps {
    title: string,
    body: string,
}

export default function (props:ICardProps) : JSX.Element{

    return <div className="card">
        <div className="card__title">
            {props.title}
        </div>
        <div className="card__body">
            {props.body}
        </div>
    </div>
}