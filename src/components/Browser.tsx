import React from 'react';
import styled from 'styled-components';
import {Vertical} from './Layout';
import Renderer from './Renderer';
import Toolbar from './Toolbar';
import * as Client from '../utils/Client';

export default function Browser() {
  let [content, setContent] = React.useState('');

  const navigate = React.useCallback((url: string) => {
    Client.preflight(url).then((res: any) => {
      if (res == 'A') {
        Client.pull(url).then((res: any) => {
          setContent(res.toString());
        }).catch(() => {
          setContent(`t\tOdin Error E\nh1\tOdin Error E\np\tServer request error`);
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
  }, []);

  const back = React.useCallback(() => {
    console.log("back");
  }, []);

  const next = React.useCallback(() => {
    console.log("next");
  }, []);

  return (
    <Container>
      <Toolbar
        onNavigate={navigate}
        canBack={false}
        onBack={back}
        canNext={false}
        onNext={next}
      />
      <Renderer content={content}/>
    </Container>
  );
}

const Container = styled(Vertical)`
  flex: 1;
  background-color: #fafafa;
  user-select: none;
  overflow: hidden;
`;
