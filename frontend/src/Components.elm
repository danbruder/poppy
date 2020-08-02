module Components exposing (layout)

import Browser exposing (Document)
import Generated.Route as Route exposing (Route)
import Html exposing (..)
import Html.Attributes as Attr exposing (class, href, style)


layout : { page : Document msg } -> Document msg
layout { page } =
    { title = page.title
    , body =
        [ div [ class "column spacing--large pad--medium container h--fill" ]
            [ navbar
            , div [ class "column", style "flex" "1 0 auto" ] page.body
            , footer
            ]
        ]
    }


navbar : Html msg
navbar =
    div [] []


footer : Html msg
footer =
    div [] []
