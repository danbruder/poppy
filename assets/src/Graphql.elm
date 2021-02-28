module GraphqlClient exposing (makeRequest)

import Graphql.Http
import RemoteData


makeRequest toMsg query =
    query
        |> Graphql.Http.queryRequest "http://localhost:8080/graphql"
        |> Graphql.Http.send (RemoteData.fromResult >> toMsg)
