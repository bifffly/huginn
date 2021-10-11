import React from 'react';
import styled from 'styled-components';
import {Vertical} from './Layout';
import Display from './Display';
import Toolbar from './Toolbar';

export default function Browser() {
  let [url, setUrl] = React.useState("");

  const navigate = (navUrl: string) => {
    setUrl(navUrl);
    console.log(url);
  };

  const back = () => {
    console.log("back");
  };

  const next = () => {
    console.log("next");
  };

  return (
    <Container>
      <Toolbar
        url={url}
        setUrl={setUrl}
        onNavigate={navigate}
        canBack={true}
        onBack={back}
        canNext={true}
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
