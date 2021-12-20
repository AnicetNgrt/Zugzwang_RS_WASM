import { Switch, Route } from 'react-router-dom'
import { Helmet } from 'react-helmet'

import './styles/index.css'
import favicon from '../public/favicon.png'

import Home from './pages/Home'
import About from './pages/About'
import Posts from './pages/Posts'
import NotFound from './pages/404'

type AppProps = {
   greet?: (name: string) => void
}

export default function App(props: AppProps): JSX.Element {
   if (props.greet) props.greet("Anicet")
   
   return (
      <>
         <Helmet>
            <meta charSet='utf-8' />
            <title>Zugzwang</title>
            <link rel='icon' type='image/png' href={favicon} />
         </Helmet>
         <Switch>
            <Route exact path='/' component={Home} />
            <Route path='/about' component={About} />
            <Route path='/posts' component={Posts} />
            <Route path='*' component={NotFound} />
         </Switch>
      </>
   )
}
