import React from 'react';
import styled from 'styled-components';
import {Vertical} from './Layout';
import Renderer from './Renderer';
import Toolbar from './Toolbar';
import * as Client from '../utils/Client';

export default function Browser(props: {
  history: string[]
}) {
  let {history} = props;
  let [pos, setPos] = React.useState(-1);
  let [content, setContent] = React.useState('');
  let [addr, setAddr] = React.useState('');

  const navigate = React.useCallback((url: string, modifyHistory: boolean) => {
    console.log(`navigate modifyHistory: ${modifyHistory}`);
    Client.preflight(url).then((res: any) => {
      if (res == 'A') {
        Client.pull(url).then((res: any) => {
          setContent(res.toString());
        }).catch(() => {
          setContent(`t\tOdin Error D\nh1\tOdin Error E\np\tServer request error`);
        });
      }
      else if (res == 'B') {
        setContent(`t\tOdin Error B\nh1\tOdin Error B\np\tFile not found`);
      }
      else if (res == 'C') {
        setContent(`t\tOdin Error C\nh1\tOdin Error C\np\tMalformed request`);
      }
    }).catch(() => {
      setContent(`t\tOdin Error D\nh1\tOdin Error D\np\tServer request error`);
    });
    if (modifyHistory) {
      if (pos + 1 < history.length) {
        history.length = pos + 1;
      }
      history.push(url);
      setPos(pos + 1);
    }
    setAddr(url);

    console.log(history);
    console.log(pos + 1);
  }, [pos]);

  const back = React.useCallback(() => {
    console.log("back");
    navigate(history[pos - 1], false);
    setPos(pos - 1);
  }, [pos]);

  const next = React.useCallback(() => {
    console.log("next");
    navigate(history[pos + 1], false);
    setPos(pos + 1);
  }, [pos]);

  let canBack = pos > 0;
  let canNext = pos < history.length - 1;

  return (
    <Container>
      <Toolbar
        addr={addr}
        setAddr={setAddr}
        onNavigate={navigate}
        canBack={canBack}
        onBack={back}
        canNext={canNext}
        onNext={next}
      />
      <Renderer content={content} navigate={navigate}/>
    </Container>
  );
}

const Container = styled(Vertical)`
  flex: 1;
  background-color: #fafafa;
  user-select: none;
  overflow: hidden;
`;
