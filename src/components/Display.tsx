import React from 'react';
import * as Client from '../utils/Client';
import Renderer from './Renderer';

export default function Display(props: {
  url: string
}) {
  let [content, setContent] = React.useState('');

  Client.preflight(props.url).then((res: any) => {
    console.log(res.toString());
    Client.pull(props.url).then((res: any) => {
      setContent(res.toString());
    });
  });

  return (
    <Renderer content={content}/>
  );
}
