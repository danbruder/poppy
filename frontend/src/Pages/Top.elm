module Pages.Top exposing (Flags, Model, Msg, page)

import Html exposing (..)
import Page exposing (Document, Page)


type alias Flags =
    ()


type alias Model =
    ()


type alias Msg =
    Never


page : Page Flags Model Msg
page =
    Page.static
        { view = view
        }


view : Document Msg
view =
    { title = "Top"
    , body = [ viewBody ]
    }


viewBody : Html Msg
viewBody =
    div [] [ text "Poppy photos" ]
