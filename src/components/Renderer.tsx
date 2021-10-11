import React from 'react';
import styled from 'styled-components';

export default function Renderer(props: {
  content: string
}) {
  let lines = props.content.split('\n');

  let content = [];
  let items = [];
  for (const line of lines) {
    let args = line.split('\t');
    let type = args.splice(0, 1)[0];

    if (type === 't') {
      document.title = "Huginn -- " + args[0];
    }
    if (type === 'h1') {
      content.push(<h1>{args[0]}</h1>);
    }
    else if (type === 'h2') {
      content .push(<h2>{args[0]}</h2>);
    }
    else if (type === 'h3') {
      content.push(<h3>{args[0]}</h3>);
    }
    else if (type === 'p') {
      content.push(<p>{args[0]}</p>);
    }
    else if (type === 'r') {
      content.push(<a href=''>{args[0]}</a>);
      content.push(<br/>);
    }
    else if (type === 'lbr') {
      content.push(<br/>);
    }
    else if (type === 'lsep') {
      content.push(<hr/>);
    }
    else if (type === 'unel') {
      content.push(<ul>{items}</ul>);
      items = [];
    }
    else if (type === 'unbl') {
      content.push(<ol>{items}</ol>);
      items = [];
    }
    else if (type === 'it') {
       items.push(<li>{args[0]}</li>);
    }
  }

  return (
    <Container>
      {content}
    </Container>
  );
}

const Container = styled.div`
  min-width: 664px;
  height: 100%;
  padding: 24px;
  overflow: hidden scroll;
  font-family: "SF Mono", Menlo, Monaco, monospace;
  font-size: 12px;
`;
