import SuccotashLogo from "./assets/logo.png";
import Card from "./components/Card";
import Counter from "./components/Counter";
import RecipeButton from "./components/RecipeButton";

function App() {
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
                    <h1>Legendary. Succotash</h1>
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
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                    <div className="column is-3">
                        <Card title="Tool 1" body="This tool will do a thing" />
                    </div>
                </div>
            </main>
        </>
    );
}

export default App;
