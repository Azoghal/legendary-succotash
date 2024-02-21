import { useCallback, useState } from "react";
import { newSpotifyExampleClient } from "../services/spotifyExample";

interface IExampleCardProps {
    title: string;
}

export default function ExampleCard(props: IExampleCardProps): JSX.Element {
    const [popularity, setPopularity] = useState<number>(-1);
    const [artist, setArtist] = useState("");

    const [bob, setBob] = useState("fella");

    const getArtistPopularity = useCallback(() => {
        console.log(
            `Lets go ask the backend for the popularity of the band: ${artist}`
        );

        newSpotifyExampleClient()
            .get(artist)
            .then((resp) => {
                setPopularity(resp.popularity);
            })
            .catch((e) => console.log(e));
    }, [artist]);

    return (
        <div className="card">
            <div className="card__title">{props.title}</div>
            <div className="card__body">
                <div>
                    <input
                        type="text"
                        onChange={(e) => setArtist(e.target.value)}
                        value={artist}
                    />
                    {artist}
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
