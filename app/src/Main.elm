module Main exposing (main)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation exposing (Key)
import Css as C
import Html.Styled exposing (..)
import Html.Styled.Attributes exposing (..)
import Html.Styled.Events exposing (..)
import Http exposing (Error, jsonBody)
import Json.Decode as D exposing (Decoder)
import Json.Encode as E
import Url exposing (Url)


type alias Todo =
    { title : String
    , description : String
    , edit : Bool
    }


encodeTodo todo =
    E.object
        [ ( "title", E.string todo.title )
        , ( "description", E.string todo.description )
        ]


decodeTodo : Decoder Todo
decodeTodo =
    D.map3 Todo
        (D.field "title" D.string)
        (D.field "description" D.string)
        (D.succeed False)


decodeTodos =
    D.list decodeTodo


type Msg
    = NoOp
    | GotTodos (Result Error (List Todo))
    | AddTodo
    | SaveTodo Todo
    | UpdateTodo String


getTodos : Cmd Msg
getTodos =
    Http.get
        { url = "http://localhost:8080/api/fetch"
        , expect = Http.expectJson GotTodos decodeTodos
        }


saveTodo : Todo -> Cmd Msg
saveTodo todo =
    Http.post
        { url = "http://localhost:8080/api/create"
        , expect = Http.expectJson GotTodos decodeTodos
        , body = jsonBody <| encodeTodo todo
        }


type alias Model =
    { todos : List Todo }



-- VIEW


view : Model -> Document Msg
view model =
    { title = "Todo App"
    , body =
        [ toUnstyled <|
            div
                [ css
                    [ C.displayFlex
                    , C.flexDirection C.column
                    ]
                ]
                (button
                    [ onClick AddTodo ]
                    [ text "+" ]
                    :: List.map
                        renderTodo
                        model.todos
                )
        ]
    }



-- RENDER TODO


renderTodo todo =
    if todo.edit then
        div []
            [ input [ onInput UpdateTodo ] []
            , button [ onClick <| SaveTodo todo ] [ text "Save" ]
            ]

    else
        div []
            [ text todo.title ]



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotTodos (Ok res) ->
            ( { todos = res }, Cmd.none )

        GotTodos (Err err) ->
            ( model, Cmd.none )

        AddTodo ->
            ( { model
                | todos = { title = "", description = "", edit = True } :: model.todos
              }
            , Cmd.none
            )

        SaveTodo todo ->
            ( model, saveTodo todo )

        UpdateTodo title ->
            ( { model
                | todos =
                    List.map
                        (\t ->
                            if t.edit then
                                { t | title = title }

                            else
                                t
                        )
                        model.todos
              }
            , Cmd.none
            )

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
