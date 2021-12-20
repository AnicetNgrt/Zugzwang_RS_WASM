import React from 'react'

interface Props {
    children?: React.ReactNode;
}

export default function Page(props: Props): JSX.Element {
   return (
      <div
         className='relative w-screen min-h-screen bg-slate-900'
      >
         {props.children}
      </div>
   )
}
