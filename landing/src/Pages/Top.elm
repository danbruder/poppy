module Pages.Top exposing (Model, Msg, page)

import Generated.Params as Params
import Html exposing (..)
import Html.Attributes exposing (..)
import Spa.Page
import Utils.Spa exposing (Page)


type alias Model =
    ()


type alias Msg =
    Never


page : Page Params.Top Model Msg model msg appMsg
page =
    Spa.Page.static
        { title = always "Poppy Photos"
        , view = always view
        }



-- VIEW


view : Html Msg
view =
    div [ class "p-8 bg-gray-200" ]
        [ hero
        ]


hero : Html Msg
hero =
    div [ class "flex justify-center items center w-full" ]
        [ div [ class "p-6 bg-white max-w-md rounded shadow-lg" ] [ img [ src "https://via.placeholder.com/800x600.jpg" ] [] ]
        , div [ class "w-1/2 p-6 " ]
            [ h1 [ class "flex text-4xl text-blue-800" ]
                [ text "Keep your memories safe"
                ]
            , p [ class "text-gray-700" ]
                [ text "Poppy Photos was designed from the ground up to make managing personal photos as simple as possible while keeping you and your loved ones safe."
                ]
            ]
        ]
