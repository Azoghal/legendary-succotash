import React from 'react';
import succotash from './resources/succotash.png';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <p>
        </p>
        <a
          className="App-link"
          href="https://en.wikipedia.org/wiki/Succotash"
          target="_blank"
          rel="noopener noreferrer"
        >
          Legendary. Succotash.
        </a>
        <img src={succotash} alt="Succotash"/>
      </header>
    </div>
  );
}

export default App;
