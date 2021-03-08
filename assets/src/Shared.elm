module Shared exposing
    ( Flags
    , Model
    , Msg
    , init
    , subscriptions
    , update
    , view
    )

import Browser.Navigation exposing (Key)
import Html exposing (..)
import Html.Attributes exposing (class, href, src)
import Spa.Document exposing (Document)
import Spa.Generated.Route as Route
import Url exposing (Url)



-- INIT


type alias Flags =
    ()


type alias Model =
    { url : Url
    , key : Key
    }


init : Flags -> Url -> Key -> ( Model, Cmd Msg )
init flags url key =
    ( Model url key
    , Cmd.none
    )



-- UPDATE


type Msg
    = ReplaceMe


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ReplaceMe ->
            ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view :
    { page : Document msg, toMsg : Msg -> msg }
    -> Model
    -> Document msg
view { page, toMsg } model =
    { title = page.title
    , body =
        [ div [ class "" ]
            [ header [ class "w-full border-b border-gray-300 flex justify-between" ]
                [ div [ class "p-6 flex items-center font-bold " ]
                    [ a [ class "mx-2 link", href (Route.toString Route.Top) ] [ text "Poppy Photos" ]
                    ]
                , div [ class "p-6 flex items-center" ]
                    [ a [ class "mx-2 link", href (Route.toString Route.Top) ] [ text "Albums" ]
                    , a [ class "mx-2 link", href (Route.toString Route.Upload) ] [ text "Upload" ]
                    , img [ class "mx-2 w-8 h-8 rounded-full", src "https://danbruder.com/js/../static/myface-d07c1438362080b352d2ef803d3b3f8c.jpeg" ] []
                    ]
                ]
            , div [ class "" ] page.body
            ]
        ]
    }
