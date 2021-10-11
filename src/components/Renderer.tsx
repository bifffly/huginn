import React from 'react';
import styled from 'styled-components';

export default function Renderer(props: {
  content: string
}) {
  let lines = props.content.split('\n');
  console.log(lines);

  return (
    <Container>
      {lines.map(line => (
        <OdinBlock line={line}/>
      ))}
    </Container>
  );
}

export function OdinBlock(props: {
  line: string
}) {
  let args = props.line.split('\t');
  let type = args.splice(0, 1)[0];

  return (
    <div>
      {type === 'h1' ? <h1>{args[0]}</h1> : null}
      {type === 'h2' ? <h2>{args[0]}</h2> : null}
      {type === 'h3' ? <h3>{args[0]}</h3> : null}
      {type === 'p' ? <p>{args[0]}</p> : null}
      {type === 'lbr' ? <br/> : null}
      {type === 'lsep' ? <hr/> : null}
    </div>
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
