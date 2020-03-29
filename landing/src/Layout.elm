module Layout exposing (view)

import Components.Flower
import Generated.Routes as Routes exposing (Route, routes)
import Html exposing (..)
import Html.Attributes exposing (..)
import Svg exposing (path, svg)
import Svg.Attributes exposing (d, viewBox)
import Utils.Spa as Spa


view : Spa.LayoutContext msg -> Html msg
view { page, route } =
    div [ class "app" ]
        [ viewHeader route
        , page
        ]


viewHeader : Route -> Html msg
viewHeader currentRoute =
    nav [ class "flex items-center justify-between flex-wrap bg-blue-500 p-6" ]
        [ div [ class "flex items-center flex-shrink-0 text-white mr-6" ]
            [ div [ class "mr-2" ] [ Components.Flower.view ]
            , span [ class "font-semibold text-xl tracking-tight" ]
                [ text "Poppy Photos" ]
            ]
        , div [ class "block lg:hidden" ]
            [ button [ class "flex items-center px-3 py-2 border rounded text-blue-200 border-blue-400 hover:text-white hover:border-white" ]
                [ svg [ Svg.Attributes.class "fill-current h-3 w-3", viewBox "0 0 20 20", attribute "xmlns" "http://www.w3.org/2000/svg" ]
                    [ node "title"
                        []
                        [ text "Menu" ]
                    , path [ d "M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" ]
                        []
                    ]
                ]
            ]
        , div [ class "w-full block flex-grow lg:flex lg:items-center lg:w-auto" ]
            [ div [ class "text-sm lg:flex-grow" ]
                [ a [ class "block mt-4 lg:inline-block lg:mt-0 text-blue-200 hover:text-white mr-4", href "#responsive-header" ]
                    [ text "What is it?" ]
                , a [ class "block mt-4 lg:inline-block lg:mt-0 text-blue-200 hover:text-white mr-4", href "#responsive-header" ]
                    [ text "Pricing" ]
                , a [ class "block mt-4 lg:inline-block lg:mt-0 text-blue-200 hover:text-white", href "#responsive-header" ]
                    [ text "Blog      " ]
                ]
            , div []
                [ a [ class "inline-block text-sm px-4 py-2 leading-none border rounded text-white border-white hover:border-transparent hover:text-blue-500 hover:bg-white mt-4 lg:mt-0", href "#" ]
                    [ text "Notify me" ]
                ]
            ]
        ]


viewLink : Route -> ( String, Route ) -> Html msg
viewLink currentRoute ( label, route ) =
    if currentRoute == route then
        span
            [ class "link link--active" ]
            [ text label ]

    else
        a
            [ class "link"
            , href (Routes.toPath route)
            ]
            [ text label ]
