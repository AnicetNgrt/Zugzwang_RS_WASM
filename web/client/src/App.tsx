import { Switch, Route } from 'react-router-dom'
import { Helmet } from 'react-helmet'

import './styles/index.css'
import favicon from '../public/favicon.png'

import GamePage from './pages/Game'
import NotFound from './pages/404'

type AppProps = {
   zzg?: any
}

export default function App(props: AppProps): JSX.Element {   
   return (
      <>
         <Helmet>
            <meta charSet='utf-8' />
            <title>Zugzwang</title>
            <link rel='icon' type='image/png' href={favicon} />
         </Helmet>
         <Switch>
            <Route exact path='/' render={() => <GamePage zzg={props.zzg}/>} />
            <Route path='*' component={NotFound} />
         </Switch>
      </>
   )
}
