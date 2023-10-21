/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

// Package main implements a server for Greeter service.
package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"log"
	"net"

	tash "court/gen/prototash"

	"google.golang.org/grpc"
)

var (
	port = flag.Int("port", 50051, "The server port")
)

type server struct {
	tash.UnimplementedServotashServer
}

type recipe struct {
	ingredients  []string
	instructions []string
	cookingTime  int
}

func listRecipes() []recipe {
	bbcSuccotash := recipe{
		ingredients: []string{
			"4 sweetcorn cobs",
			"1 tbsp olive oil",
			"2 garilc cloves, crushed",
			"100g frozen baby broad bean",
			"1 red chilli, deseeded and chopped",
			"large handful basil, chopped",
			"large handful mint, chopped",
			"1-2 tsp sherry vinegar",
		},
		instructions: []string{
			"Use a knife to cut down the length of the sweetcorn to remove the kernels.",
			"Heat the oil in a large lidded pan.",
			"Cook the kernels and garlic over a medium heat for 5 mins, stirring all the time.",
			"Add the frozen beans to the pan, cover and cook, stirring every so often, for another 4-5 mins or until the beans are cooked through.",
			"Turn off the heat and add the chilli, herbs and vinegar.",
			"Taste and add seasoning, if you like.",
		},
		cookingTime: 30,
	}

	easySuccotash := recipe{
		ingredients: []string{
			"2 tablespoons olive oil",
			"1/2 white onion, minced",
			"4 cups frozen corn (or fresh corn cut off the cob)",
			"2 cups frozen lima beans",
			"1 red pepper, finely diced",
			"1 pint cherry tomatoes, halved",
			"1 teaspoon garlic powder",
			"½ teaspoon smoked paprika",
			"½ teaspoon ground sage",
			"1 ¼ teaspoon kosher salt, plus more to taste",
			"Fresh ground black pepper",
			"1 tablespoon salted butter (or olive oil)",
			"2 tablespoons minced fresh parsley, optional",
		},
		instructions: []string{
			"Heat the olive oil in a large skillet over medium high heat.",
			"Add the onion and cook for 2 minutes, until translucent.",
			"Add the corn, beans, red pepper, tomatoes, garlic powder,  smoked paprika, dried sage, salt, and the fresh ground black pepper.",
			"Cook, stirring occasionally until all vegetables are tender and nearly cooked, about 5 to 6 minutes.",
			"Stir in the salted butter and parsley (if using) and cook 1 minute more, until the butter is melted.",
			"Taste and add the additional salt if desired.",
			"Serve warm.",
		},
		cookingTime: 20,
	}

	return []recipe{easySuccotash, bbcSuccotash}
}

func formatRecipe(rec recipe) []string {
	res := []string{
		fmt.Sprintf("Cooking Time: %d", rec.cookingTime),
	}
	res = append(res, rec.ingredients...)
	res = append(res, rec.instructions...)
	res = append(res, "Que Aproveche!")
	return res
}

func (s *server) Recipe(ctx context.Context, req *tash.RecipeRequest) (*tash.RecipeReply, error) {

	recipes := listRecipes()

	var resp *tash.RecipeReply
	for _, r := range recipes {
		if r.cookingTime <= int(req.MaxTime) {
			resp = &tash.RecipeReply{Instructions: formatRecipe(r)}
			return resp, nil
		}
	}

	return nil, errors.New("no recipes that quick")
}

func main() {
	flag.Parse()
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	tash.RegisterServotashServer(s, &server{})
	log.Printf("server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
