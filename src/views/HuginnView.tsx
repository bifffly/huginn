import React from 'react';
import Browser from '../components/Browser';

export default function App() {
  let [pos, setPos] = React.useState(0);

  return (
    <Browser history={[]}/>
  );
}
