import React from 'react'
import Page from '../components/Page'

type Props = {
   zzg?: any
}

export default function Game(props: Props) {
   return (
      <Page>
         <div className="
            absolute
            h-full w-full 
            flex items-center justify-center
            bg-white
         ">
            { props.zzg ? (
               <h1>Wasm loaded</h1>
            ) : (
               <h1>Wasm not loaded</h1>
            ) }
         </div>
      </Page>
   )
}
