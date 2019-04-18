module Main exposing (main)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation exposing (Key)
import Html.Styled exposing (..)
import Http exposing (Error)
import Json.Decode as D exposing (Decoder)
import Url exposing (Url)


type alias Todo =
    { title : String
    , description : String
    }


decodeTodo : Decoder Todo
decodeTodo =
    D.map2 Todo
        (D.field "title" D.string)
        (D.field "description" D.string)


decodeTodos =
    D.list decodeTodo


type Msg
    = NoOp
    | GotTodos (Result Error (List Todo))


getTodos : Cmd Msg
getTodos =
    Http.get
        { url = "http://localhost:8080/api/fetch"
        , expect = Http.expectJson GotTodos decodeTodos
        }


type alias Model =
    { todos : List Todo }


view : Model -> Document Msg
view model =
    { title = "Todo App"
    , body =
        [ toUnstyled <|
            div []
                (List.map
                    (\t -> div [] [ text t.title ])
                    model.todos
                )
        ]
    }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotTodos (Ok res) ->
            ( { todos = res }, Cmd.none )

        GotTodos (Err err) ->
            ( model, Cmd.none )

        _ ->
            ( model, Cmd.none )


onUrlRequest : UrlRequest -> Msg
onUrlRequest urlRequest =
    NoOp


onUrlChange : Url -> Msg
onUrlChange url =
    NoOp


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


type alias Flags =
    String


init : Flags -> Url -> Key -> ( Model, Cmd Msg )
init flags url key =
    ( { todos = [] }, getTodos )


main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = onUrlChange
        , onUrlRequest = onUrlRequest
        }
