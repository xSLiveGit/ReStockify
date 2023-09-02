import { useState } from 'react';

export default function MyApp() {
  const [count1, setCount1] = useState(0);
  const [count2, setCount2] = useState(0);

  function handleClick() {
    setCount1(count1 + 1);
    setCount2(count2 + 2);
  }

  return (
    <div>
      <h1>Counters that update separately</h1>
      <MyButton count1={count1} count2={count2} onClick={handleClick} />
      <MyButton count1={count1} count2={count2} onClick={handleClick} />
    </div>
  );
}

function MyButton({count1, count2, onClick}) {
  return (
    <button onClick={onClick}>
      Clicked {count1} and {count2} times 
    </button>
  );
}