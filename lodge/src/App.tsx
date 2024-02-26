import Landing from "./components/Landing";
import * as Router from "react-router";
import TestNotLanding from "./components/TestNotLanding";

function App() {
    return (
        <>
            <Router.Routes>
                <Router.Route path="/" Component={Landing} />
                <Router.Route path="/notlanding" Component={TestNotLanding} />
            </Router.Routes>
        </>
    );
}

export default App;
