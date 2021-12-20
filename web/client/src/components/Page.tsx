import React from 'react'

interface Props {
    children?: React.ReactNode;
}

export default function Page(props: Props): JSX.Element {
   return (
      <div
         className='relative w-screen min-h-screen'
      >
         {props.children}
      </div>
   )
}
