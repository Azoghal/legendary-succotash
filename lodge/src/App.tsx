import SuccotashLogo from "./assets/logo.png";
import Card from "./components/Card";
import Counter from "./components/Counter";
import RecipeButton from "./components/RecipeButton";

function App() {
    return (
        <>
            <div>
                <div>
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
                </div>
                <h1>Legendary. Succotash</h1>
                <Counter val={0} />
                <RecipeButton />
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
            </div>
        </>
    );
}

export default App;
