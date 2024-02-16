import { useCallback, useState } from "react";

interface ICardProps {
    title: string;
}

export default function (props: ICardProps): JSX.Element {
    const [popularity, setPopularity] = useState<number>(-1);
    const [artist, setArtist] = useState("");

    const getArtistPopularity = useCallback(() => {
        setPopularity(0);
    }, []);

    return (
        <div className="card">
            <div className="card__title">{props.title}</div>
            <div className="card__body">
                <div>
                    <input
                        onChange={(e) => setArtist(e.target.value)}
                        value={artist}
                    ></input>
                </div>
                <div>Popularity: {popularity == -1 ? "..." : popularity}</div>
            </div>
            <div className="card__quick-action">
                <button
                    className="c-btn c-btn__alternate"
                    onClick={getArtistPopularity}
                >
                    Get Artist Popularity
                </button>
            </div>
        </div>
    );
}
