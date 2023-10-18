import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('it renders succotash', () => {
  render(<App />);
  const imageElement = screen.getByAltText("Succotash");
  expect(imageElement).toBeInTheDocument();
});
