module Pages.Top exposing (Model, Msg, Params, page)

import Data.Photo as Photo exposing (Photo)
import Html exposing (..)
import Html.Attributes exposing (..)
import Shared
import Spa.Document exposing (Document)
import Spa.Page as Page exposing (Page)
import Spa.Url as Url exposing (Url)


page : Page Params Model Msg
page =
    Page.application
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        , save = save
        , load = load
        }



-- INIT


type alias Params =
    ()


type alias Model =
    { photos : List Photo
    }


init : Shared.Model -> Url Params -> ( Model, Cmd Msg )
init shared { params } =
    ( { photos = Photo.dummy }, Cmd.none )



-- UPDATE


type Msg
    = ReplaceMe


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ReplaceMe ->
            ( model, Cmd.none )


save : Model -> Shared.Model -> Shared.Model
save model shared =
    shared


load : Shared.Model -> Model -> ( Model, Cmd Msg )
load shared model =
    ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Document Msg
view model =
    { title = "Top"
    , body =
        [ viewPhotos model
        ]
    }


viewPhotos : Model -> Html Msg
viewPhotos model =
    List.map viewPhoto model.photos
        |> div [ class "flex flex-wrap m-1 pt-4 px-4" ]


viewPhoto : Photo -> Html Msg
viewPhoto photo =
    div [ class "h-56 m-1" ]
        [ img [ class "h-full ", src photo.url ] []
        ]
