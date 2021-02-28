module Queries.PhotosList exposing (..)

import Data.Photo exposing (Photo)
import Graphql.Http
import Graphql.Operation exposing (RootQuery)
import Graphql.OptionalArgument exposing (..)
import Graphql.SelectionSet as SelectionSet exposing (SelectionSet, with)
import GraphqlClient
import Juniper.Object.Photo as Photo
import Juniper.Query as Query
import RemoteData exposing (RemoteData)


type alias Response =
    RemoteData (Graphql.Http.Error Payload) Payload


type alias Payload =
    { photos : List Photo }


photoSelection =
    SelectionSet.succeed Photo
        |> with Photo.uri


query : SelectionSet Payload RootQuery
query =
    SelectionSet.map Payload
        (Query.photos photoSelection)



-- WALL ARGS


run toMsg =
    GraphqlClient.makeRequest toMsg query
