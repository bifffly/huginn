import React from 'react';
import Renderer from './Renderer';

export default function Display(props: {
  url: string
}) {
  return (
    <Renderer content={`h1\tTest\nlbr\nh2\tTest\nh3\tTest\nlsep\np\tPlaintext`}/>
  );
}
