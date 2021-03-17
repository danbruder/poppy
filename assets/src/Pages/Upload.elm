module Pages.Upload exposing (Model, Msg, Params, page)

import Browser.Navigation as Nav
import File exposing (File)
import File.Select as Select
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as D
import Spa.Document exposing (Document)
import Spa.Generated.Route as Route
import Spa.Page as Page exposing (Page)
import Spa.Url as Url exposing (Url)
import Task
import Utils.Route


page : Page Params Model Msg
page =
    Page.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- INIT


type alias Params =
    ()


type alias Model =
    { hover : Bool
    , previews : List String
    , files : List File
    , key : Nav.Key
    }


init : Url Params -> ( Model, Cmd Msg )
init { params, key } =
    ( Model False [] [] key, Cmd.none )



-- UPDATE


type Msg
    = Pick
    | DragEnter
    | DragLeave
    | GotFiles File (List File)
    | GotPreviews (List String)
    | UploadedFile (Result Http.Error ())


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Pick ->
            ( model
            , Select.files [ "image/*" ] GotFiles
            )

        DragEnter ->
            ( { model | hover = True }
            , Cmd.none
            )

        DragLeave ->
            ( { model | hover = False }
            , Cmd.none
            )

        GotFiles file files ->
            ( { model | hover = False, files = file :: files }
            , Cmd.batch
                [ Task.perform GotPreviews <|
                    Task.sequence <|
                        List.map File.toUrl (file :: files)
                , (file :: files) |> upload
                ]
            )

        GotPreviews urls ->
            ( { model | previews = urls }
            , Cmd.none
            )

        UploadedFile _ ->
            ( { model | files = [], previews = [] }
            , Utils.Route.navigate model.key Route.Top
            )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Document Msg
view model =
    { title = "Upload"
    , body =
        [ viewDropzone model
        ]
    }


viewDropzone : Model -> Html Msg
viewDropzone model =
    div
        [ style "border"
            (if model.hover then
                "6px dashed purple"

             else
                "6px dashed #ccc"
            )
        , style "border-radius" "20px"
        , style "width" "480px"
        , style "margin" "100px auto"
        , style "padding" "40px"
        , style "display" "flex"
        , style "flex-direction" "column"
        , style "justify-content" "center"
        , style "align-items" "center"
        , hijackOn "dragenter" (D.succeed DragEnter)
        , hijackOn "dragover" (D.succeed DragEnter)
        , hijackOn "dragleave" (D.succeed DragLeave)
        , hijackOn "drop" dropDecoder
        ]
        [ if List.length model.files == 0 then
            button [ onClick Pick, class "bg-indigo-600 text-white p-1 px-3 rounded cursor-pointer" ] [ text "Upload Images" ]

          else
            button [ class "bg-indigo-600 opacity-50 text-white p-1 px-3 rounded cursor-pointer" ] [ text "Uploading..." ]
        ]


dropDecoder : D.Decoder Msg
dropDecoder =
    D.at [ "dataTransfer", "files" ] (D.oneOrMore GotFiles File.decoder)


hijackOn : String -> D.Decoder msg -> Attribute msg
hijackOn event decoder =
    preventDefaultOn event (D.map hijack decoder)


hijack : msg -> ( msg, Bool )
hijack msg =
    ( msg, True )


upload : List File -> Cmd Msg
upload files =
    Http.request
        { method = "POST"
        , headers = []
        , url = "/upload"
        , body =
            files
                |> List.map (Http.filePart "files[]")
                |> Http.multipartBody
        , expect = Http.expectWhatever UploadedFile
        , timeout = Nothing
        , tracker = Nothing
        }
