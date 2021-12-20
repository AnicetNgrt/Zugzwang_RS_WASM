import React, { useEffect } from 'react'

type Props = {
   zzg?: any
}

export default function Game(props: Props) {
   return (
      <>
         <div className="
            absolute
            h-full w-full 
            flex items-center justify-center
         ">
            { props.zzg ? (
               <h2 className='text-green-600'>
                  WASM INCLUDED
               </h2>
            ) : (
               <h1 className='text-red-600'>
                  NO WASM
               </h1>
            ) }
         </div>
      </>
   )
}
