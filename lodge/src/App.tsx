import { t } from "i18next";
import SuccotashLogo from "./assets/logo.png";
import Card from "./components/Card";
import Counter from "./components/Counter";
import RecipeButton from "./components/RecipeButton";

function App() {
    //TODO remove
    const bob = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    return (
        <>
            <header className="c-header">
                <div className="c-header-left">
                    <a
                        href="https://en.wikipedia.org/wiki/Succotash"
                        target="_blank"
                    >
                        <img
                            src={SuccotashLogo}
                            className="logo"
                            alt="Succotash Logo"
                        />
                    </a>
                    <h1>{t("title.succotash")}</h1>
                </div>
                <div className="c-header-right">
                    <Counter val={0} />
                    <RecipeButton />
                </div>
            </header>
            <main className="c-page">
                {/* https://bulma.io/documentation/columns/options/ */}
                {/* The above has a cool example where we can have wider or narrower things in each row */}
                <div className="columns is-multiline is-centered card-holder">
                    {bob.map((i) => (
                        <div className="column is-3">
                            <Card
                                title={t("tool.card.title", { count: i })}
                                body={t("tool.card.desc")}
                            />
                        </div>
                    ))}
                </div>
            </main>
        </>
    );
}

export default App;
