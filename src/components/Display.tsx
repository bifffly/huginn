import React from 'react';

export default function Display(props: {
  url: string
}) {
  return (
    <div>
      {props.url}
    </div>
  );
}
