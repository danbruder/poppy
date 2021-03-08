module GraphqlClient exposing (makeRequest)

import Graphql.Http
import RemoteData


makeRequest toMsg query =
    query
        |> Graphql.Http.queryRequest "/graphql"
        |> Graphql.Http.send (RemoteData.fromResult >> toMsg)
