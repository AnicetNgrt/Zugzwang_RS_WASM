import React from 'react'
import Counter from '../components/Counter'

const Home: React.FC = () => {
   return (
      <>
         <div className='wrapper'>
            <h1 className="text-2xl text-red-500">
               Hello from
               <br />
               ZUGZWANG
            </h1>
            <p>React (SSR) project powered by actix backend and graphql</p>
            <Counter />
         </div>
      </>
   )
}

export default Home
