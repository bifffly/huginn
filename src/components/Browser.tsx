import React from 'react';
import styled from 'styled-components';
import {Vertical} from './Layout';
import Display from './Display';
import Toolbar from './Toolbar';

export default function Browser() {
  let [url, setUrl] = React.useState("");

  const navigate = React.useCallback((navUrl: string) => {
    setUrl(navUrl);
  }, [url]);

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
      <Display url={url}/>
    </Container>
  );
}

const Container = styled(Vertical)`
  flex: 1;
  background-color: #fafafa;
  user-select: none;
  overflow: hidden;
`;
