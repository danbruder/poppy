module Pages.Top exposing (Model, Msg, Params, page)

import Data.Photo as Photo exposing (Photo)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Queries.PhotosList
import RemoteData as RD
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
    , selectedPhoto : Maybe Photo
    }


init : Shared.Model -> Url Params -> ( Model, Cmd Msg )
init shared { params } =
    ( { photos = [], selectedPhoto = Nothing }
    , Queries.PhotosList.run GotPhotos
    )



-- UPDATE


type Msg
    = GotPhotos Queries.PhotosList.Response
    | ClickedPhoto Photo
    | ClickedSelectedPhoto


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotPhotos (RD.Success payload) ->
            ( { model | photos = payload.photos }, Cmd.none )

        ClickedPhoto photo ->
            ( { model | selectedPhoto = Just photo }, Cmd.none )

        ClickedSelectedPhoto ->
            ( { model | selectedPhoto = Nothing }, Cmd.none )

        _ ->
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
        , viewBlackroom model
        ]
    }


viewPhotos : Model -> Html Msg
viewPhotos model =
    List.map viewPhoto model.photos
        |> div [ class "flex flex-wrap m-1 pt-4 px-4" ]


viewPhoto : Photo -> Html Msg
viewPhoto photo =
    div [ class "h-56 m-1", onClick <| ClickedPhoto <| photo ]
        [ img [ class "h-full ", src photo.url ] []
        ]


viewBlackroom : Model -> Html Msg
viewBlackroom model =
    model.selectedPhoto |> Maybe.map viewPhotoInBlackroom |> Maybe.withDefault (span [] [])


viewPhotoInBlackroom : Photo -> Html Msg
viewPhotoInBlackroom photo =
    div [ class "inset-0 fixed p-20 bg-gray-900 bg-opacity-95", onClick ClickedSelectedPhoto ]
        [ img [ class "h-full w-full", src photo.url ] []
        ]
